package br.com.gabryel.adventofcode.y2022

typealias Coordinate = Pair<Int, Int>

fun Coordinate.x() = first
fun Coordinate.y() = second

infix fun Coordinate.distanceTo(other: Coordinate) = (x() - other.x()) to (y() - other.y())
operator fun Coordinate.plus(other: Coordinate) = (x() + other.x()) to (y() + other.y())

fun Coordinate.getAdjacent() = Direction.entries.map { this + it.vector }

val ZERO = 0 to 0

enum class Direction(val vector: Coordinate) {
    R(1 to 0), L(-1 to 0), U(0 to 1), D(0 to -1)
}
