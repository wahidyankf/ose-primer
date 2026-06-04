package com.demobektkt.infrastructure

import com.demobektkt.domain.Attachment
import com.demobektkt.infrastructure.repositories.AttachmentRepository
import com.demobektkt.infrastructure.repositories.CreateAttachmentRequest
import com.demobektkt.infrastructure.tables.AttachmentsTable
import java.time.Instant
import java.util.UUID
import org.jetbrains.exposed.v1.core.ResultRow
import org.jetbrains.exposed.v1.core.eq
import org.jetbrains.exposed.v1.jdbc.deleteWhere
import org.jetbrains.exposed.v1.jdbc.insert
import org.jetbrains.exposed.v1.jdbc.selectAll

class ExposedAttachmentRepository : AttachmentRepository {
  private fun rowToAttachment(row: ResultRow): Attachment =
    Attachment(
      id = row[AttachmentsTable.id],
      expenseId = row[AttachmentsTable.expenseId],
      filename = row[AttachmentsTable.filename],
      contentType = row[AttachmentsTable.contentType],
      size = row[AttachmentsTable.size],
      data = row[AttachmentsTable.data],
      createdAt = row[AttachmentsTable.createdAt],
    )

  override suspend fun create(request: CreateAttachmentRequest): Attachment = ioTransaction {
    val id =
      AttachmentsTable.insert {
          it[expenseId] = request.expenseId
          it[filename] = request.filename
          it[contentType] = request.contentType
          it[size] = request.size
          it[data] = request.data
          it[createdAt] = Instant.now()
        }[AttachmentsTable.id]
    AttachmentsTable.selectAll()
      .where { AttachmentsTable.id eq id }
      .map { rowToAttachment(it) }
      .single()
  }

  override suspend fun findById(id: UUID): Attachment? = ioTransaction {
    AttachmentsTable.selectAll()
      .where { AttachmentsTable.id eq id }
      .map { rowToAttachment(it) }
      .singleOrNull()
  }

  override suspend fun findAllByExpense(expenseId: UUID): List<Attachment> = ioTransaction {
    AttachmentsTable.selectAll()
      .where { AttachmentsTable.expenseId eq expenseId }
      .map { rowToAttachment(it) }
  }

  override suspend fun delete(id: UUID): Boolean = ioTransaction {
    val deleted = AttachmentsTable.deleteWhere { AttachmentsTable.id eq id }
    deleted > 0
  }
}
