package br.com.gabryel.adventofcode.util

typealias Array2D<T> = Array<Array<T>>

typealias CharArray2D = Array<CharArray>

typealias IntArray2D = Array<IntArray>

typealias LongArray2D = Array<LongArray>

typealias Adjacency<T> = Triple<Coordinate, T, Direction>

fun List<String>.toCharArray2D() = Array(size) { y -> CharArray(first().length) { x -> this[y][x] } }

fun CharArray2D.findAdjacent(coordinate: Coordinate) = Direction.entries.mapNotNull { dir ->
    val newPosition = coordinate + dir.vector
    val content = this.getOrNull(newPosition)
        ?: return@mapNotNull null

    Adjacency(newPosition, content, dir)
}

fun CharArray2D.findAdjacentWithNulls(coordinate: Coordinate) = Direction.entries.map { dir ->
    val newPosition = coordinate + dir.vector
    Adjacency(newPosition, this.getOrNull(newPosition), dir)
}

fun IntArray2D.findAdjacent(coordinate: Coordinate) = Direction.entries.mapNotNull { dir ->
    val newPosition = coordinate + dir.vector
    val content = this.getOrNull(newPosition)
        ?: return@mapNotNull null

    Adjacency(newPosition, content, dir)
}

operator fun <T> Array2D<T>.contains(coord: Coordinate) = this[coord] != null

operator fun <T> Array2D<T>.set(coord: Coordinate, item: T) {
    this[coord.y()][coord.x()] = item
}

operator fun CharArray2D.set(coord: Coordinate, item: Char) {
    this[coord.y()][coord.x()] = item
}

operator fun LongArray2D.set(coord: Coordinate, item: Long) {
    this[coord.y()][coord.x()] = item
}

operator fun IntArray2D.set(coord: Coordinate, item: Int) {
    this[coord.y()][coord.x()] = item
}

inline fun <reified T> Array2D<T?>.mapValues(map: (T) -> T) =
    map { it.map { it?.let { map(it) } }.toTypedArray() }.toTypedArray()

fun <T> Array2D<T>.getOrNull(coord: Coordinate) =
    getOrNull(coord.y())?.getOrNull(coord.x())

fun CharArray2D.getOrNull(coord: Coordinate) =
    getOrNull(coord.y())?.getOrNull(coord.x())

fun IntArray2D.getOrNull(coord: Coordinate) =
    getOrNull(coord.y())?.getOrNull(coord.x())

operator fun <T> Array2D<T>.get(coord: Coordinate) = this[coord.y()][coord.x()]

operator fun CharArray2D.get(coord: Coordinate) = this[coord.y()][coord.x()]

operator fun LongArray2D.get(coord: Coordinate) = this[coord.y()][coord.x()]

operator fun IntArray2D.get(coord: Coordinate) = this[coord.y()][coord.x()]

fun <T> Array2D<T>.getAll() = asSequence().flatMap { it.asSequence() }

fun <T> Array2D<T>.getAllEntries() = asSequence().flatMapIndexed { y, row ->
    row.asSequence().mapIndexedNotNull { x, item -> item?.let { (x to y) to item } }
}

fun LongArray2D.getAll() = asSequence().flatMap { it.asSequence() }

fun IntArray2D.getAll() = asSequence().flatMap { it.asSequence() }

fun CharArray2D.findFirst(content: Char) = asSequence().withIndex().firstNotNullOf { (y, item) ->
    item.withIndex().firstOrNull { (_, value) -> value == content }?.let { (x) -> x to y }
}

fun CharArray2D.findFirstOrNull(content: Char) = asSequence().withIndex().firstNotNullOfOrNull { (y, item) ->
    item.withIndex().firstOrNull { (_, value) -> value == content }?.let { (x) -> x to y }
}

inline fun <reified T> array2D(width: Int, height: Int) = Array(height) { arrayOfNulls<T?>(width) }