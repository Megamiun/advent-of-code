package br.com.gabryel.adventofcode.util

enum class Direction(
    val vector: Coordinate,
    val inverse: () -> Direction,
    val clockwise: () -> Direction,
    val counterClockwise: () -> Direction,
    val horizontal: Boolean
) {
    RIGHT(1 to 0, { LEFT }, { DOWN }, { UP }, true),
    DOWN(0 to 1, { UP }, { LEFT }, { RIGHT }, false),
    LEFT(-1 to 0, { RIGHT }, { UP }, { DOWN }, true),
    UP(0 to -1, { DOWN }, { RIGHT }, { LEFT }, false);

    companion object {
        fun findByInitial(initial: String) = Direction.entries.first { it.name.startsWith(initial) }
    }
}
