package com.demobektkt.infrastructure

import com.demobektkt.domain.Page
import com.demobektkt.domain.User
import com.demobektkt.domain.UserStatus
import com.demobektkt.infrastructure.repositories.CreateUserRequest
import com.demobektkt.infrastructure.repositories.UpdateUserPatch
import com.demobektkt.infrastructure.repositories.UserRepository
import com.demobektkt.infrastructure.tables.UsersTable
import java.time.Instant
import java.util.UUID
import org.jetbrains.exposed.v1.core.eq
import org.jetbrains.exposed.v1.core.like
import org.jetbrains.exposed.v1.core.or
import org.jetbrains.exposed.v1.jdbc.andWhere
import org.jetbrains.exposed.v1.jdbc.insert
import org.jetbrains.exposed.v1.jdbc.selectAll
import org.jetbrains.exposed.v1.jdbc.update

class ExposedUserRepository : UserRepository {
  private fun rowToUser(row: org.jetbrains.exposed.v1.core.ResultRow): User =
    User(
      id = row[UsersTable.id],
      username = row[UsersTable.username],
      email = row[UsersTable.email],
      displayName = row[UsersTable.displayName],
      passwordHash = row[UsersTable.passwordHash],
      role = row[UsersTable.role],
      status = row[UsersTable.status],
      failedLoginAttempts = row[UsersTable.failedLoginAttempts],
      createdAt = row[UsersTable.createdAt],
      updatedAt = row[UsersTable.updatedAt],
    )

  override suspend fun findByUsername(username: String): User? = ioTransaction {
    UsersTable.selectAll()
      .where { UsersTable.username eq username }
      .map { rowToUser(it) }
      .singleOrNull()
  }

  override suspend fun findById(id: UUID): User? = ioTransaction {
    UsersTable.selectAll().where { UsersTable.id eq id }.map { rowToUser(it) }.singleOrNull()
  }

  override suspend fun findByEmail(email: String): User? = ioTransaction {
    UsersTable.selectAll().where { UsersTable.email eq email }.map { rowToUser(it) }.singleOrNull()
  }

  override suspend fun create(request: CreateUserRequest): User = ioTransaction {
    val now = Instant.now()
    val id =
      UsersTable.insert {
          it[username] = request.username
          it[email] = request.email
          it[displayName] = request.displayName
          it[passwordHash] = request.passwordHash
          it[role] = request.role
          it[status] = UserStatus.ACTIVE
          it[failedLoginAttempts] = 0
          it[createdAt] = now
          it[updatedAt] = now
        }[UsersTable.id]
    UsersTable.selectAll().where { UsersTable.id eq id }.map { rowToUser(it) }.single()
  }

  override suspend fun update(id: UUID, patch: UpdateUserPatch): User? = ioTransaction {
    val now = Instant.now()
    UsersTable.update({ UsersTable.id eq id }) {
      patch.displayName?.let { v -> it[displayName] = v }
      patch.passwordHash?.let { v -> it[passwordHash] = v }
      patch.status?.let { v -> it[status] = v }
      patch.failedLoginAttempts?.let { v -> it[failedLoginAttempts] = v }
      patch.role?.let { v -> it[role] = v }
      it[updatedAt] = now
    }
    UsersTable.selectAll().where { UsersTable.id eq id }.map { rowToUser(it) }.singleOrNull()
  }

  override suspend fun incrementFailedLogins(id: UUID): Int = ioTransaction {
    val user = UsersTable.selectAll().where { UsersTable.id eq id }.map { rowToUser(it) }.single()
    val newCount = user.failedLoginAttempts + 1
    UsersTable.update({ UsersTable.id eq id }) {
      it[failedLoginAttempts] = newCount
      it[updatedAt] = Instant.now()
    }
    newCount
  }

  override suspend fun resetFailedLogins(id: UUID) {
    ioTransaction {
      UsersTable.update({ UsersTable.id eq id }) {
        it[failedLoginAttempts] = 0
        it[updatedAt] = Instant.now()
      }
    }
  }

  override suspend fun findAll(page: Int, pageSize: Int, searchFilter: String?): Page<User> =
    ioTransaction {
      var query = UsersTable.selectAll()
      if (searchFilter != null) {
        query =
          query.andWhere {
            (UsersTable.username like "%$searchFilter%") or
              (UsersTable.email like "%$searchFilter%")
          }
      }
      val total = query.count()
      val items =
        query
          .orderBy(UsersTable.createdAt to org.jetbrains.exposed.v1.core.SortOrder.ASC)
          .limit(pageSize)
          .offset(((page - 1) * pageSize).toLong())
          .map { rowToUser(it) }
      Page(data = items, total = total, page = page, pageSize = pageSize)
    }
}
