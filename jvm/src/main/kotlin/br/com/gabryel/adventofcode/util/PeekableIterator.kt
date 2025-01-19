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

    inline fun nextIfMatches(matches: (T) -> Boolean) =
        if (hasNext() && matches(peek())) next() else null
}

fun <T> Iterable<T>.peekingIterator() = PeekableIterator(this.iterator())