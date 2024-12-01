package br.com.gabryel.adventofcode.y2022

private val numberRegex = "\\d+".toRegex()

typealias ContainerQuantityTaker = List<Char>.(Int) -> List<Char>
typealias ContainerTaker = List<Char>.() -> List<Char>
typealias Stacks = List<List<Char>>
typealias Instructions = List<Triple<Int, Int, Int>>

fun main() {
    val stacksAndInstructions = findStacksAndInstructions()

    stacksAndInstructions.restack("9000") { quantity -> takeLast(quantity).reversed() }
    stacksAndInstructions.restack("9001") { quantity -> takeLast(quantity) }
}

private fun Pair<Stacks, Instructions>.restack(name: String, takeContainers: ContainerQuantityTaker) {
    val (stacks, instructions) = this
    val result = instructions.fold(stacks) { acc, (quantity, start, destination)  ->
        acc.move(start, destination) { takeContainers(quantity) }
    }

    println("[$name] At Top: ${result.joinToString("") { it.last().toString() }}")
}

private fun Stacks.move(origin: Int, destination: Int, takeContainers: ContainerTaker): Stacks {
    val originStack = this[origin]
    val destinationStack = this[destination]

    val content = originStack.takeContainers()

    return replace(origin, originStack.dropLast(content.size))
        .replace(destination, destinationStack + content)
}

private fun <E> List<E>.replace(position: Int, item: E) =
    take(position) + item + takeLast((size - position) - 1)

private fun findStacksAndInstructions() =
    getLines().getStacks() to getLines().getInstructions()

private fun Sequence<String>.getStacks(): Stacks {
    val reversed = toList().reversed()

    val cratePositions = reversed.first()
        .mapIndexedNotNull { index, c -> if (c == ' ') null else index }

    return cratePositions.map { index ->
        reversed.drop(1).mapNotNull { line -> line.getOrNull(index).takeIf { it != ' ' } }
    }
}

private fun Sequence<String>.getInstructions() = map { line ->
    val (quantity, origin, destination) = numberRegex.findAll(line).map { number -> number.value.toInt() }.toList()
    Triple(quantity, origin - 1, destination - 1)
}.toList()
