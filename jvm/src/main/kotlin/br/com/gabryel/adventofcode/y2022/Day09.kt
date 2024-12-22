package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.*
import java.util.concurrent.TimeUnit.NANOSECONDS
import kotlin.math.absoluteValue
import kotlin.system.measureNanoTime

private val moveCache = mutableMapOf<Pair<Coordinate, Coordinate>, Coordinate>()

fun main() {
    val commands = getCommands()
    val unwrappedCommands = commands.flatMap { (direction, steps) -> (0 until steps).map { direction } }

    unwrappedCommands.runWithSize(2)
    unwrappedCommands.runWithSize(10)
}

private fun List<Direction>.runWithSize(size: Int) {
    val nano = measureNanoTime {
        val rope = fold(Rope.withSize(size)) { current, direction -> current + direction.vector }
        println("Spaces the rope #$size passed through: ${rope.getLast().headSpaces.size}")
    }

    println("It took ${NANOSECONDS.toMillis(nano)}ms to run with $size nodes")
    println()
}

private fun getCommands() = getLinesFromSystemIn {
    val (direction, steps) = it.split(" ")
    Direction.valueOf(direction) to steps.toInt()
}.toList()

private sealed class Rope {
    abstract fun follow(previous: Coordinate): Rope

    companion object {
        fun withSize(size: Int): KnotRope =
            if (size <= 1) KnotRope(tail = EmptyRope)
            else KnotRope(tail = withSize(size - 1))
    }
}

private data object EmptyRope: Rope() {
    override fun follow(previous: Coordinate) = this
}

private data class KnotRope(
    val head: Coordinate = ZERO,
    val tail: Rope,
    val headSpaces: Set<Coordinate> = setOf(ZERO)
): Rope() {

    override fun follow(previous: Coordinate) = this + vectorTo(previous)

    fun getLast(): KnotRope = if (tail is KnotRope) tail.getLast() else this

    operator fun plus(vector: Coordinate): KnotRope {
        if (vector == ZERO) return this

        val newHead = head + vector
        return copy(head = newHead, tail = tail.follow(newHead), headSpaces = headSpaces + newHead)
    }

    private fun vectorTo(previous: Coordinate) = moveCache.computeIfAbsent(previous to head) {
        val distance = previous distanceTo head
        distance.squaresToMove()
    }

    private fun Coordinate.squaresToMove() =
        if (x().absoluteValue > 1 || y().absoluteValue > 1)
            x().squaresToMove() to y().squaresToMove()
        else ZERO

    private fun Int.squaresToMove() = when {
        this == 0 -> 0
        this > 0 -> 1
        else -> -1
    }
}
