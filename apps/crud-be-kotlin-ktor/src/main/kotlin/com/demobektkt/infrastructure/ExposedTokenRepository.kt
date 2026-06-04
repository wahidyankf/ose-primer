package com.demobektkt.infrastructure

import com.demobektkt.infrastructure.repositories.TokenRecord
import com.demobektkt.infrastructure.repositories.TokenRepository
import com.demobektkt.infrastructure.repositories.TokenType
import com.demobektkt.infrastructure.tables.RefreshTokensTable
import com.demobektkt.infrastructure.tables.RevokedTokensTable
import java.time.Instant
import java.util.UUID
import org.jetbrains.exposed.v1.core.eq
import org.jetbrains.exposed.v1.jdbc.insert
import org.jetbrains.exposed.v1.jdbc.selectAll
import org.jetbrains.exposed.v1.jdbc.update

class ExposedTokenRepository : TokenRepository {
  override suspend fun revoke(jti: String, userId: UUID, tokenType: TokenType, expiresAt: Instant) {
    ioTransaction {
      val exists =
        RevokedTokensTable.selectAll().where { RevokedTokensTable.jti eq jti }.count() > 0
      if (!exists) {
        RevokedTokensTable.insert {
          it[RevokedTokensTable.jti] = jti
          it[RevokedTokensTable.userId] = userId
          it[revokedAt] = Instant.now()
        }
      }
    }
  }

  override suspend fun isRevoked(jti: String): Boolean = ioTransaction {
    RevokedTokensTable.selectAll().where { RevokedTokensTable.jti eq jti }.count() > 0
  }

  override suspend fun revokeAllForUser(userId: UUID) {
    ioTransaction {
      RefreshTokensTable.update({ RefreshTokensTable.userId eq userId }) { it[revoked] = true }
    }
  }

  override suspend fun findByJti(jti: String): TokenRecord? = ioTransaction {
    RevokedTokensTable.selectAll()
      .where { RevokedTokensTable.jti eq jti }
      .map { row ->
        TokenRecord(
          jti = row[RevokedTokensTable.jti],
          userId = row[RevokedTokensTable.userId],
          tokenType = TokenType.ACCESS,
          expiresAt = Instant.now(),
          revokedAt = row[RevokedTokensTable.revokedAt],
        )
      }
      .singleOrNull()
  }
}
