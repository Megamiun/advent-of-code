package br.com.gabryel.adventofcode.y2023.d15

import br.com.gabryel.adventofcode.readLines

typealias IndexMap = Array<MutableList<Pair<String, Int>>>

fun main() {
    listOf("sample", "input").forEach { file ->
        val instructions = readLines(2023, 15, file).map { it.split(",") }.first()

        println("[Instruction Hash][$file] ${instructions.sumOf { it.calculateHash() }}")
        println("[Focal Power     ][$file] ${instructions.findFocalPower()}")
    }
}

private fun List<String>.findFocalPower(): Int {
    val boxes: IndexMap = Array(256) { mutableListOf() }
    boxes.apply(this)

    return boxes.indices.sumOf { boxIndex ->
        val boxValue = boxIndex + 1
        boxes[boxIndex].mapIndexed { lensIndex, item -> boxValue * (lensIndex + 1) * item.second }.sum()
    }
}

private fun IndexMap.apply(instructions: List<String>) {
    val parsedInstructions = instructions.map {
        val (len, power) = it.split("[-=]".toRegex())
        len to power.takeIf { it.isNotBlank() }?.toInt()
    }

    parsedInstructions.forEach { (lens, power) ->
        if (power == null) remove(lens)
        else update(lens, power)
    }
}

private fun IndexMap.remove(lens: String) {
    this[lens.calculateHash()].removeIf { (element) -> element == lens }
}

private fun IndexMap.update(lens: String, power: Int) {
    val box = this[lens.calculateHash()]
    val lensPosition = box.indexOfFirst { it.first == lens }

    if (lensPosition == -1)
        box.add(lens to power)
    else
        box[lensPosition] = lens to power
}

private fun String.calculateHash(acc: Int = 0, index: Int = 0): Int {
    if (index > this.lastIndex) return acc
    return calculateHash(((acc + this[index].code) * 17) % 256, index + 1)
}
