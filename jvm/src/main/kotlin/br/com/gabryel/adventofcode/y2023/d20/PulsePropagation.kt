package br.com.gabryel.adventofcode.y2023.d20

import br.com.gabryel.adventofcode.readLines
import java.util.concurrent.atomic.AtomicInteger

fun main() {
    listOf("sample", "sample2", "input").forEach { file ->
        val spec = readLines(2023, 20, file)

        println("[Signal Multiplication][$file] ${spec.getSignalMultiplication()}")
        println("[RX Button Press      ][$file] ${spec.getFirstCallToRx()}")
    }
}

private fun List<String>.getSignalMultiplication(): Int {
    val connectors = this
        .plus("button -> broadcaster")
        .associate(Connection::from)
    connectors.registerOnConjuction()

    val button = connectors["button"]!!
    val signalsSent = mapOf(true to AtomicInteger(), false to AtomicInteger())

    repeat(1000) {
        val signals = ArrayDeque<Triple<String, String, Boolean>>().apply {
            addAll(button.receiveSignal("player", false))
        }

        while (signals.isNotEmpty()) {
            val (from, to, signal) = signals.removeFirst()
            signalsSent[signal]!!.incrementAndGet()

            val toConnector = connectors[to] ?: continue

            signals.addAll(toConnector.receiveSignal(from, signal))
        }
    }

    return signalsSent[true]!!.get() * signalsSent[false]!!.get()
}

private fun List<String>.getFirstCallToRx(): Int {
    val connectors = associate(Connection::from)
    connectors.registerOnConjuction()

    if (connectors.values.flatMap { it.outputs }.none { it == "rx" })
        return -1

    val broadcaster = connectors["broadcaster"]!!
    generateSequence(1) { it + 1 }.forEach { buttonClick ->
        val signals = ArrayDeque<Triple<String, String, Boolean>>().apply {
            addAll(broadcaster.receiveSignal("button", false))
        }

        while (signals.isNotEmpty()) {
            val (from, to, signal) = signals.removeFirst()
            val toConnector = connectors[to]

            if (toConnector != null) {
                signals.addAll(toConnector.receiveSignal(from, signal))
                continue
            }

            if (to == "rx" && !signal)
                return buttonClick
        }
    }

    return -1
}

private fun Map<String, Connection>.registerOnConjuction() {
    forEach { (name, connector) ->
        connector.outputs
            .mapNotNull { this[it] }
            .filterIsInstance<Conjunction>()
            .forEach { it.registerInput(name) }
    }
}

private sealed class Connection {

    abstract val outputs: List<String>
    abstract fun receiveSignal(fromConnector: String, high: Boolean): List<Triple<String, String, Boolean>>

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
}

private class FlipFlop(val name: String, override val outputs: List<String>, private var memoryHigh: Boolean = false): Connection() {
    override fun receiveSignal(fromConnector: String, high: Boolean): List<Triple<String, String, Boolean>> {
        if (high) return emptyList()
        memoryHigh = !memoryHigh

        return outputs.map { Triple(name, it, memoryHigh) }
    }
}

private class Conjunction(val name: String, override val outputs: List<String>): Connection() {
    private val inputs = mutableMapOf<String, Boolean>()

    fun registerInput(name: String) {
        inputs[name] = false
    }

    override fun receiveSignal(fromConnector: String, high: Boolean): List<Triple<String, String, Boolean>> {
        inputs[fromConnector] = high

        return if (inputs.all { it.value }) outputs.map { Triple(name, it, false) }
        else outputs.map { Triple(name, it, true) }
    }
}
