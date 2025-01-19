package br.com.gabryel.adventofcode.y2022.d06

fun extractPositionAndMarker(lines: List<String>, size: Int): Int {
    val markerIndex = lines.first()
        .windowedSequence(size)
        .indexOfFirst { letters -> letters.toSet().size == size }

    return markerIndex + size
}
