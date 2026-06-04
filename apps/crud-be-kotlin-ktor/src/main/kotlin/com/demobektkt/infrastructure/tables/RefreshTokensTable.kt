package com.demobektkt.infrastructure.tables

import org.jetbrains.exposed.v1.core.Table
import org.jetbrains.exposed.v1.core.java.javaUUID
import org.jetbrains.exposed.v1.javatime.timestamp

object RefreshTokensTable : Table("refresh_tokens") {
  val id = javaUUID("id").autoGenerate()
  val userId = javaUUID("user_id")
  val tokenHash = varchar("token_hash", 255).uniqueIndex()
  val expiresAt = timestamp("expires_at")
  val revoked = bool("revoked").default(false)
  val createdAt = timestamp("created_at")
  override val primaryKey = PrimaryKey(id)
}
