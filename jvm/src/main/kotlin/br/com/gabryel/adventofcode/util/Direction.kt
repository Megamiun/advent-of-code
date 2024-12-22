package br.com.gabryel.adventofcode.util

enum class Direction(val vector: Coordinate) {
    RIGHT(1 to 0), LEFT(-1 to 0), UP(0 to 1), DOWN(0 to -1)
}
