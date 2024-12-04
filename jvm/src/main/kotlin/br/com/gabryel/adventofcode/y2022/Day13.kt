package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.y2022.Packet.Num
import br.com.gabryel.adventofcode.y2022.Packet.PacketArray
import java.util.*

fun main() {
    val inOrder = getPacketPairs()
        .withIndex()
        .filter { (_, content) -> content.first <= content.second }
        .map { it.index + 1 }
        .sum()

    println("In order: $inOrder")
}

private sealed interface Packet : Comparable<Packet> {
    data class Num(val value: Int) : Packet {
        override fun compareTo(other: Packet) = when (other) {
            is Num -> value.compareTo(other.value)
            is PacketArray -> PacketArray(listOf(this)).compareTo(other)
        }
    }

    data class PacketArray(val values: List<Packet>) : Packet {
        override fun compareTo(other: Packet): Int = when (other) {
            is Num -> compareTo(PacketArray(listOf(other)))
            is PacketArray -> {
                values.zip(other.values)
                    .map { (l, r) -> l.compareTo(r) }
                    .firstOrNull { it != 0 }
                    ?: values.size.compareTo(other.values.size)
            }
        }
    }
}

private fun getPacketPairs() = getGroupsOfLines()
    .map { (l, r) -> toPacket(l) to toPacket(r) }

private fun toPacket(line: String): Packet {
    val queue = LinkedList(line.toList())
    queue.remove()
    return toPacket(queue)
}

private fun toPacket(queue: Queue<Char>): Packet = generateSequence {
    when (queue.peek()) {
        '[' -> queue.remove().let { toPacket(queue) }
        ']' -> queue.remove().let { null }
        ',' -> queue.remove().let { toPacket(queue) }
        null -> null
        else -> extractNumber(queue)
    }
}.toList().let(::PacketArray)

private fun extractNumber(queue: Queue<Char>): Num {
    val number = generateSequence {
        when (queue.peek()) {
            ']', ',' -> null
            else -> queue.remove()
        }
    }.takeWhile { it.isDigit() }
        .joinToString("")

    return Num(number.toInt())
}
