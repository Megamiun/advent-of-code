package br.com.gabryel.adventofcode.y2023.d19

data class Piece(val x: Int, val m: Int, val a: Int, val s: Int) {
    fun getAttribute(attribute: Char) = when (attribute) {
        'x' -> x
        'm' -> m
        'a' -> a
        's' -> s
        else -> throw IllegalArgumentException("No attribute named '$attribute'")
    }
}

fun parsePieces(lines: List<String>) = lines.map {
    val (x, m, a, s) = pieceBreaker.findAll(it).map { it.value.toInt() }.toList()
    Piece(x, m, a, s)
}
