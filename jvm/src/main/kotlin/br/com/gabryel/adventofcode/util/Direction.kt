package br.com.gabryel.adventofcode.util

enum class Direction(
    val vector: Coordinate,
    val inverse: () -> Direction,
    val clockwise: () -> Direction,
    val counterClockwise: () -> Direction
) {
    RIGHT(1 to 0, { LEFT }, { DOWN }, { UP }),
    DOWN(0 to 1, { UP }, { LEFT }, { RIGHT }),
    LEFT(-1 to 0, { RIGHT }, { UP }, { DOWN }),
    UP(0 to -1, { DOWN }, { RIGHT }, { LEFT })
}
