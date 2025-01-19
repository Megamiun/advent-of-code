package br.com.gabryel.adventofcode.y2022.d05

private val numberRegex = "\\d+".toRegex()

private typealias Stack = List<Char>
private typealias StackAmountTaker = Stack.(Int) -> Stack
private typealias StackTaker = Stack.() -> Stack

fun restackOneByOne(groups: List<List<String>>) =
    groups.restack { quantity -> takeLast(quantity).reversed() }

fun restackFully(groups: List<List<String>>) =
    groups.restack { quantity -> takeLast(quantity) }

private fun List<List<String>>.restack(takeContainers: StackAmountTaker): String {
    val stacks = this[0].getStacks()
    val instructions = this[1].getInstructions()

    return instructions.fold(stacks) { acc, (quantity, start, destination) ->
        acc.move(start, destination) { takeContainers(quantity) }
    }.joinToString("") { it.last().toString() }
}

private fun List<Stack>.move(origin: Int, destination: Int, takeContainers: StackTaker): List<Stack> {
    val originStack = this[origin]
    val destinationStack = this[destination]

    val moved = originStack.takeContainers()

    return replace(origin, originStack.dropLast(moved.size))
        .replace(destination, destinationStack + moved)
}

private fun List<Stack>.replace(position: Int, item: Stack): List<Stack> =
    take(position) + listOf(item) + takeLast((size - position) - 1)

private fun List<String>.getStacks(): List<Stack> {
    val reversed = reversed()

    val cratePositions = reversed.first()
        .mapIndexedNotNull { index, c -> if (c == ' ') null else index }

    return cratePositions.map { index ->
        reversed.drop(1).mapNotNull { line -> line.getOrNull(index).takeIf { it != ' ' } }
    }
}

private fun List<String>.getInstructions() = map { line ->
    val (quantity, origin, destination) = numberRegex.findAll(line).map { number -> number.value.toInt() }.toList()
    Triple(quantity, origin - 1, destination - 1)
}
