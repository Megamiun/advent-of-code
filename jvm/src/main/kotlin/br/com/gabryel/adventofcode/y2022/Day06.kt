package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.getLinesFromSystemIn

fun main() {
    getLinesFromSystemIn().forEach { line ->
        val (packetMarkerEnd, packetMarker) = extractPositionAndMarker(line, 4)
        val (messageMarkerEnd, messageMarker) = extractPositionAndMarker(line, 14)

        println(line)
        println("\t$packetMarker - $packetMarkerEnd")
        println("\t$messageMarker - $messageMarkerEnd")
        println("---------")
    }
}

private fun extractPositionAndMarker(line: String, size: Int): Pair<Int, String> {
    val markerWindows = line.windowedSequence(size)
    val markerIndex = markerWindows.indexOfFirst { letters -> letters.toSet().size == size }
    val markerIndexEnd = markerIndex + size
    val marker = line.substring(markerIndex, markerIndexEnd)
    return markerIndexEnd to marker
}
