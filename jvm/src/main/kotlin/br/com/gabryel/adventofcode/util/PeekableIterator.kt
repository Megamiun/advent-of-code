package br.com.gabryel.adventofcode.util

import java.util.function.Consumer

class PeekableIterator<T>(private val iterator: Iterator<T>) : Iterator<T> by iterator {
    private var cached: T? = null

    override fun forEachRemaining(action: Consumer<in T>) {
        val next = cached
        if (next != null) {
            cached = null
            action.accept(next)
        }
        forEachRemaining(action)
    }

    override fun hasNext() =
        cached != null || iterator.hasNext()

    override fun next(): T {
        val next = cached
            ?: return iterator.next()

        cached = null
        return next
    }

    fun peek(): T {
        if (cached != null)
            return cached ?: throw IllegalStateException("Cached result was lost on reading")

        val next = iterator.next()
        cached = next
        return next
    }

    inline fun hasNextMatches(matches: (T) -> Boolean) =
        hasNext() && matches(peek())
}

fun <T> Iterable<T>.peekingIterator() = PeekableIterator(this.iterator())

fun <T> Iterable<T>.takeWhileInclusive(predicate: (T) -> Boolean) =
    peekingIterator().takeWhileInclusive(predicate)

fun <T> PeekableIterator<T>.takeWhileInclusive(predicate: (T) -> Boolean) = sequence {
    yieldAll(takeWhile(predicate))

    if (hasNext()) yield(next())
}

fun <T> PeekableIterator<T>.takeWhile(predicate: (T) -> Boolean) = sequence {
    while (hasNextMatches(predicate)) {
        yield(next())
    }
}
