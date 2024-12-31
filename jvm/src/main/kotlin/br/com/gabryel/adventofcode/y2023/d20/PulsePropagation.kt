package br.com.gabryel.adventofcode.y2023.d20

import br.com.gabryel.adventofcode.util.readLines
import java.util.concurrent.atomic.AtomicInteger

fun main() {
    listOf("sample", "sample-2", "input").forEach { file ->
        val spec = readLines(2023, 20, file)

        println("[Signal Multiplication][$file] ${spec.getSignalMultiplication()}")
        println("[RX Button Press      ][$file] ${spec.getFirstCallToRx()}")
    }
}

private fun List<String>.getSignalMultiplication(): Int {
    val connectors = this
        .plus("button -> broadcaster")
        .associate(Connection::from)
    connectors.registerOnConjunction()

    val button = connectors["button"]!!
    val signalsSent = (1..1000).map {
        connectors.clickButton(button)
    }.reduce { acc, curr -> (acc.first + curr.first) to (acc.second + curr.second) }

    return signalsSent.first * signalsSent.second
}

private fun List<String>.getFirstCallToRx(): Long {
    val connectors = associate(Connection::from)
    connectors.registerOnConjunction()

    val rxPointing = connectors.entries.firstOrNull { "rx" in it.value.outputs }
        ?: return -1

    val clusters = connectors.clusterize(rxPointing.key)
    val clusterCycles = connectors.findCycles(clusters)

    return clusterCycles.mapNotNull(CycleInfo::cycleLength).leastCommonMultiplier()
}

private fun Map<String, Connection>.findCycles(clusters: Map<String, List<Connection>>): Collection<CycleInfo> {
    val broadcaster = this["broadcaster"]!!

    val clusterCycles = clusters.mapValues { (_, connections) -> CycleInfo(connections) }

    generateSequence(1L) { it + 1 }.map { buttonClick ->
        clickButton(broadcaster)

        clusterCycles.values.forEach { it.updateClosedStatus(buttonClick) }
    }.takeWhile { !clusterCycles.values.all { it.isClosed() } }.last()

    return clusterCycles.values
}

private fun Map<String, Connection>.clickButton(starter: Connection): Pair<Int, Int> {
    val signalsSent = mapOf(true to AtomicInteger(), false to AtomicInteger())
    val signals = ArrayDeque<Triple<String, String, Boolean>>().apply {
        addAll(starter.receiveSignal("player", false))
    }

    while (signals.isNotEmpty()) {
        val (from, to, signal) = signals.removeFirst()
        signalsSent[signal]!!.incrementAndGet()

        val toConnector = this[to] ?: continue

        signals.addAll(toConnector.receiveSignal(from, signal))
    }

    return (signalsSent[true]?.get() ?: 0) to (signalsSent[false]?.get() ?: 0)
}

private fun Map<String, Connection>.clusterize(key: String): Map<String, List<Connection>> =
    filter { key in it.value.outputs }.map { it.key }.associateWith { clusterRoot ->
        generateSequence(setOf(clusterRoot) to setOf<String>()) { (toVisit, visited) ->
            val newToVisit = toVisit
                .filter { it !in visited }
                .flatMap { visit -> entries.filter { visit in it.value.outputs }.map { it.key } }
                .toSet()

            newToVisit to (visited + toVisit)
        }.takeWhile { it.first.isNotEmpty() }
            .last().second.minusElement(clusterRoot)
            .mapNotNull { this[it] }
    }

private fun Map<String, Connection>.registerOnConjunction() {
    forEach { (name, connector) ->
        connector.outputs
            .mapNotNull { this[it] }
            .filterIsInstance<Conjunction>()
            .forEach { it.registerInput(name) }
    }
}

private fun List<Long>.leastCommonMultiplier() = reduce { acc, curr -> leastCommonMultiplier(acc, curr) }

private fun leastCommonMultiplier(a: Long, b: Long) = (a * b) / greatestCommonDivisor(a, b)

private fun greatestCommonDivisor(a: Long, b: Long): Long {
    if (b == 0L) return a
    return greatestCommonDivisor(b, a % b)
}

private class CycleInfo(private val connections: List<Connection>) {
    var cycleLength: Long? = null
        private set

    fun updateClosedStatus(buttonClick: Long) {
        if (cycleLength == null && connections.all { it.isResetted() }) {
            cycleLength = buttonClick
        }
    }

    fun isClosed() = cycleLength != null
}

private sealed class Connection {
    abstract val outputs: List<String>
    abstract fun receiveSignal(fromConnector: String, high: Boolean): List<Triple<String, String, Boolean>>
    abstract fun isResetted(): Boolean

    companion object {
        private val breaker = "([&%]?)(.*) -> (.*)".toRegex()

        fun from(line: String): Pair<String, Connection> {
            val (type, name, outputs) = breaker.find(line)!!.groupValues.drop(1)

            val outputsList = outputs.split(", ")

            return name to when (type) {
                "&" -> Conjunction(name, outputsList)
                "%" -> FlipFlop(name, outputsList)
                else -> Broadcaster(name, outputsList)
            }
        }
    }
}

private data class Broadcaster(val name: String, override val outputs: List<String>): Connection() {
    override fun receiveSignal(fromConnector: String, high: Boolean) = outputs.map { Triple(name, it, high) }
    override fun isResetted() = true
}

private data class FlipFlop(val name: String, override val outputs: List<String>): Connection() {
    private var memoryHigh: Boolean = false

    override fun receiveSignal(fromConnector: String, high: Boolean): List<Triple<String, String, Boolean>> {
        if (high) return emptyList()
        memoryHigh = !memoryHigh

        return outputs.map { Triple(name, it, memoryHigh) }
    }

    override fun isResetted() = !memoryHigh
}

private data class Conjunction(val name: String, override val outputs: List<String>): Connection() {
    private val inputs = mutableMapOf<String, Boolean>()

    fun registerInput(name: String) {
        inputs[name] = false
    }

    override fun receiveSignal(fromConnector: String, high: Boolean): List<Triple<String, String, Boolean>> {
        inputs[fromConnector] = high

        return if (inputs.all { it.value }) outputs.map { Triple(name, it, false) }
        else outputs.map { Triple(name, it, true) }
    }

    override fun isResetted() = inputs.values.all { !it }
}
