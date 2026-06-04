package com.demobektkt.infrastructure

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.jetbrains.exposed.v1.jdbc.JdbcTransaction
import org.jetbrains.exposed.v1.jdbc.transactions.suspendTransaction

/**
 * Runs [block] inside a suspended Exposed JDBC transaction dispatched on [Dispatchers.IO].
 *
 * Exposed 1.0.0 deprecated `newSuspendedTransaction(context)`; the replacement
 * `suspendTransaction()` no longer accepts a [kotlin.coroutines.CoroutineContext], so IO dispatch
 * is provided explicitly via [withContext]. This helper preserves the previous
 * `newSuspendedTransaction(Dispatchers.IO) { ... }` semantics in a single call site.
 */
internal suspend fun <T> ioTransaction(block: suspend JdbcTransaction.() -> T): T =
  withContext(Dispatchers.IO) { suspendTransaction(statement = block) }
