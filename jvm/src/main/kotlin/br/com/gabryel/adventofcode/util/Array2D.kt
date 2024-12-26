package br.com.gabryel.adventofcode.util

typealias Array2D<T> = Array<Array<T>>

typealias CharArray2D = Array<CharArray>

fun CharArray2D.findAdjacent(coordinate: Coordinate) =
    Direction.entries
        .map { dir -> (coordinate + dir.vector) to (this[coordinate + dir.vector] to dir) }

fun <T> Array2D<T>.findAdjacent(coordinate: Coordinate) =
    Direction.entries
        .map { dir -> (coordinate + dir.vector) to (this[coordinate + dir.vector] to dir) }

operator fun <T> Array2D<T>.contains(coord: Coordinate) = this[coord] != null

operator fun <T> Array2D<T>.get(coord: Coordinate) = getOrNull(coord.y())?.getOrNull(coord.x())

operator fun <T> Array2D<T>.set(coord: Coordinate, item: T) {
    this[coord.y()][coord.x()] = item
}

operator fun CharArray2D.get(coord: Coordinate) = getOrNull(coord.y())?.getOrNull(coord.x())

fun <T> Array2D<T>.getAll() = asSequence().flatMap { it.asSequence() }.filterNotNull()

fun <T> Array2D<T>.getAllIndexed() = asSequence().withIndex()
    .flatMap { (y, item) -> item.asSequence().withIndex().mapNotNull { (x, value) -> value?.let { (x to y) to value } } }
