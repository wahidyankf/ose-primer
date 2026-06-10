package envbackup

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// IsSecretFile returns true if a file at relPath with base filename should be
// backed up or restored. Mirrors Rust `is_secret_file`.
func IsSecretFile(base, relPath string) bool {
	if strings.HasPrefix(base, ".env") {
		return true
	}
	if base == "secrets.json" {
		return true
	}
	// *.pem / *.key / *.crt / *.pfx certificate and key files
	ext := strings.ToLower(filepath.Ext(base))
	switch ext {
	case ".pem", ".key", ".crt", ".pfx":
		return true
	}
	// Any file descended into via .secrets/ (carved out of hidden-dir skip)
	if strings.HasPrefix(relPath, ".secrets/") || strings.HasPrefix(relPath, ".secrets\\") {
		return true
	}
	// *.tfvars and inventory files — activate when IaC is added (R3/R11)
	// if strings.HasSuffix(base, ".tfvars") { return true }
	return false
}

// DefaultBackupDirName returns the default backup directory name derived from
// the repo root basename: "<repo-basename>-env-backup". Mirrors Rust
// `default_backup_dir_name`.
func DefaultBackupDirName(repoBasename string) string {
	return repoBasename + "-env-backup"
}

// Discover walks RepoRoot and returns all secret files found, including those
// skipped due to being symlinks or exceeding MaxSize. Results are sorted by
// RelPath for deterministic ordering.
func Discover(opts Options) ([]FileEntry, error) {
	if opts.MaxSize <= 0 {
		opts.MaxSize = DefaultMaxSize
	}

	skipSet := make(map[string]bool, len(opts.SkipDirs))
	for _, d := range opts.SkipDirs {
		skipSet[d] = true
	}

	var entries []FileEntry

	err := filepath.WalkDir(opts.RepoRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		base := filepath.Base(path)

		// Handle directories.
		if d.IsDir() {
			// Never skip the root itself.
			if path == opts.RepoRoot {
				return nil
			}
			// Carve out .secrets/ so its contents are discoverable.
			if base == ".secrets" {
				return nil
			}
			// Skip hidden directories (basename starts with ".").
			if strings.HasPrefix(base, ".") {
				return filepath.SkipDir
			}
			// Skip dirs in the skip set.
			if skipSet[base] {
				return filepath.SkipDir
			}
			return nil
		}

		relPath, err := filepath.Rel(opts.RepoRoot, path)
		if err != nil {
			return fmt.Errorf("compute relative path for %s: %w", path, err)
		}

		// Apply widened allowlist: .env*, secrets.json, cert files, .secrets/ contents.
		if !IsSecretFile(base, relPath) {
			return nil
		}

		// Use Lstat to detect symlinks without following them.
		fi, err := os.Lstat(path)
		if err != nil {
			return fmt.Errorf("lstat %s: %w", path, err)
		}

		if fi.Mode()&os.ModeSymlink != 0 {
			entries = append(entries, FileEntry{
				RelPath: relPath,
				AbsPath: path,
				Skipped: true,
				Reason:  "symlink",
			})
			return nil
		}

		size := fi.Size()
		if size > opts.MaxSize {
			entries = append(entries, FileEntry{
				RelPath: relPath,
				AbsPath: path,
				Size:    size,
				Skipped: true,
				Reason:  "exceeds 1 MB",
			})
			return nil
		}

		entries = append(entries, FileEntry{
			RelPath: relPath,
			AbsPath: path,
			Size:    size,
		})
		return nil
	})
	if err != nil {
		return nil, fmt.Errorf("walk %s: %w", opts.RepoRoot, err)
	}

	sort.Slice(entries, func(i, j int) bool {
		return entries[i].RelPath < entries[j].RelPath
	})

	return entries, nil
}
