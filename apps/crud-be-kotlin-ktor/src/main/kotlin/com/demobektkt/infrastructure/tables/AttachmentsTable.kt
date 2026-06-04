package com.demobektkt.infrastructure.tables

import org.jetbrains.exposed.v1.core.Table
import org.jetbrains.exposed.v1.core.java.javaUUID
import org.jetbrains.exposed.v1.javatime.timestamp

object AttachmentsTable : Table("attachments") {
  val id = javaUUID("id").autoGenerate()
  val expenseId = javaUUID("expense_id")
  val filename = varchar("filename", 255)
  val contentType = varchar("content_type", 100)
  val size = long("size")
  val data = binary("data")
  val createdAt = timestamp("created_at")
  override val primaryKey = PrimaryKey(id)
}
