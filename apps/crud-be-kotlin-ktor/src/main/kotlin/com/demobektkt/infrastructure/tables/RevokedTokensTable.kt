package com.demobektkt.infrastructure.tables

import org.jetbrains.exposed.v1.core.Table
import org.jetbrains.exposed.v1.core.java.javaUUID
import org.jetbrains.exposed.v1.javatime.timestamp

object RevokedTokensTable : Table("revoked_tokens") {
  val id = javaUUID("id").autoGenerate()
  val jti = varchar("jti", 255).uniqueIndex()
  val userId = javaUUID("user_id")
  val revokedAt = timestamp("revoked_at")
  override val primaryKey = PrimaryKey(id)
}
