package br.com.gabryel.adventofcode.y2022

private val digitsRegex = """(\d+)""".toRegex()
private val operationRegex = """([+*]) (\w+)""".toRegex()

fun main() {
    val monkeyInfo = generateSequence { getMonkeyInfo() }.toList()

    monkeyInfo.calculateMonkeyBusiness(3, 20)
    monkeyInfo.calculateMonkeyBusiness(1, 10000)
}

private fun List<List<String>>.calculateMonkeyBusiness(worryDivisor: Long, rounds: Int) {
    val mcc = map(List<String>::getDivisor).reduce(Long::times)
    val monkeys = getAllMonkeys()

    repeat(rounds) { monkeys.forEach { it.throwAll(worryDivisor, mcc) } }

    val monkeyBusinessLevel = monkeys.map { it.analyzed }.sortedDescending().take(2).reduce(Long::times)
    println("Level of monkey business with worry divisor $worryDivisor in $rounds rounds: $monkeyBusinessLevel")
}

private fun List<List<String>>.getAllMonkeys(): List<Monkey> {
    lateinit var lazyMonkeys: Lazy<List<Monkey>>
    lazyMonkeys = lazy { map { lines -> lines.createMonkey(lazyMonkeys) } }

    return lazyMonkeys.value
}

private fun List<String>.createMonkey(monkeys: Lazy<List<Monkey>>): Monkey {
    val items = extractItems()

    val throwToNextMonkey = mountThrowToNextMonkey { monkey, value -> monkeys.value[monkey].receive(value) }
    return Monkey(items, mountOperation(), throwToNextMonkey)
}

private fun List<String>.extractItems() = digitsRegex.findAll(this[1]).map { it.value.toLong() }.toMutableList()

private fun List<String>.mountThrowToNextMonkey(throwTo: (Int, Long) -> Unit): (Long) -> Unit {
    val divisibleBy = getDivisor()
    val trueMonkey = digitsRegex.find(this[4])!!.value.toInt()
    val falseMonkey = digitsRegex.find(this[5])!!.value.toInt()

    return {
        val throwToIndex = if (it % divisibleBy == 0L) trueMonkey else falseMonkey
        throwTo(throwToIndex, it)
    }
}

private fun List<String>.mountOperation(): (Long) -> Long {
    val (_, operationSignal, valueString) = operationRegex.find(this[2])!!.groups.map { it!!.value }

    val operator: (Long, Long) -> Long = when (operationSignal) {
        "*" -> Long::times
        else -> Long::plus
    }

    val value = valueString.toLongOrNull()
    return { operator(it, value ?: it) }
}

private fun List<String>.getDivisor() = digitsRegex.find(this[3])!!.value.toLong()

private class Monkey(
    private val items: MutableList<Long>,
    private val operation: (Long) -> Long,
    private val throwToNextMonkey: (Long) -> Unit
) {
    var analyzed = 0L

    fun receive(value: Long) { items += value }

    fun throwAll(worryDivisor: Long, mcc: Long) {
        items.forEach { throwItem(it, worryDivisor, mcc) }
        items.clear()
    }

    private fun throwItem(inspected: Long, worryDivisor: Long, mcc: Long) {
        analyzed++
        val newValue = (operation(inspected) / worryDivisor) % mcc
        throwToNextMonkey(newValue)
    }

}

private fun getMonkeyInfo() = getLines().toList().ifEmpty { null }
