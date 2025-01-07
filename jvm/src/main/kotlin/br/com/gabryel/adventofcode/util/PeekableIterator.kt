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
        val next = iterator.next()
        cached = next
        return next
    }
}

fun <T> Iterable<T>.peekingIterator() = PeekableIterator(this.iterator())