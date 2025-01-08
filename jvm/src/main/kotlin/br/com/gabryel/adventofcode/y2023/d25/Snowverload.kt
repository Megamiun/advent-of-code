package br.com.gabryel.adventofcode.y2023.d25

import br.com.gabryel.adventofcode.util.*
import kotlin.math.max
import kotlin.math.min

private typealias Mapping = Pair<Int, Int>

private val wordFinder = "\\w+".toRegex()

fun getProductsOfSnowverload(lines: List<String>): Int {
    val nodes = lines.flatMap { wordFinder.findAll(it).map { it.value } }
        .distinct()
        .mapIndexed { index, word -> word to index }
        .toMap()

    val connections = lines.flatMap { line ->
        val (left, others) = line.split(": ")
        val nodeLeft = nodes[left]!!
        others.split(" ").map { sortedIndices(nodeLeft, nodes[it]!!) }
    }

    return Context(connections, nodes.size).findProductOfSplitGroups()
}

private class Context(private val edges: List<Mapping>, private val numberOfNodes: Int) {
    private val connectionsMap = initializeConnectionsMap()
    private val minDistances = initializeMinDistances()
    private val paths = initializePaths()
    private val maxEdgesToCheck = 40

    fun findProductOfSplitGroups() = updatePaths().drop(3)
        .firstNotNullOf { searchCuttableCables() }

    private fun searchCuttableCables(): Int? {
        val frequency = sortEdgesByFrequency()
        val lastToCheck = min(maxEdgesToCheck, frequency.size - 1)
        val connections = connectionsMap.map { it.toMutableList() }

        (0..lastToCheck).forEach { first ->
            val wire1 = frequency[first]
            connections.disconnect(wire1)
            (first + 1..lastToCheck).forEach { second ->
                val wire2 = frequency[second]
                connections.disconnect(wire2)
                (second + 1..lastToCheck).forEach { third ->
                    val wire3 = frequency[third]
                    connections.disconnect(wire3)

                    val reachableNodes = BooleanArray(numberOfNodes).apply {
                        set(0, true)
                    }

                    val toVisit = ArrayList<Int>(numberOfNodes).apply {
                        add(0)
                    }

                    while (toVisit.isNotEmpty()) {
                        val current = toVisit.removeLast()

                        for (next in connections[current]) {
                            if (!reachableNodes[next]) {
                                reachableNodes[next] = true
                                toVisit += next
                            }
                        }
                    }

                    if (reachableNodes.any { !it })
                        return reachableNodes.count { it } * reachableNodes.count { !it }

                    connections.reconnect(wire3)
                }
                connections.reconnect(wire2)
            }
            connections.reconnect(wire1)
        }

        return null
    }

    private fun List<MutableList<Int>>.disconnect(wire: Mapping) {
        this[wire.x()] -= wire.y()
        this[wire.y()] -= wire.x()
    }

    private fun List<MutableList<Int>>.reconnect(wire: Mapping) {
        this[wire.x()] += wire.y()
        this[wire.y()] += wire.x()
    }

    private fun sortEdgesByFrequency() = paths.reversed()
        .flatMapIndexed { from, allTo -> allTo.take(from).flatten() }
        .groupingBy { it }.eachCount()
        .entries.sortedByDescending { it.value }
        .map { it.key }

    private fun updatePaths() = sequence {
        var changed = true
        while (changed) {
            changed = false

            for (start in 0 until numberOfNodes) {
                for (end in start + 1 until numberOfNodes) {
                    val startToEnd = start to end
                    var minDistance = minDistances[startToEnd]
                    var minIntermediate = end

                    for (intermediate in 0 until numberOfNodes) {
                        val toIntermediate = minDistances[start][intermediate]
                        if (toIntermediate >= minDistance) continue

                        val fromIntermediate = minDistances[intermediate][end]
                        if (fromIntermediate >= minDistance) continue

                        val candidate = toIntermediate + fromIntermediate
                        if (minDistance > candidate) {
                            minDistance = candidate
                            minIntermediate = intermediate
                        }
                    }

                    if (updateRecursive(start, minIntermediate, end, minDistance, 7))
                        changed = true
                }

                yield(Unit)
            }
        }
    }

    private fun updateRecursive(start: Int, intermediate: Int, end: Int, distance: Int, levels: Int): Boolean {
        if (minDistances[start][end] <= distance) return false

        val fullPath = paths[start][intermediate] + paths[intermediate][end]
        minDistances[end][start] = distance
        minDistances[start][end] = distance
        paths[end][start] = fullPath
        paths[start][end] = fullPath

        if (levels == 0) return true

        for (next in 0 until numberOfNodes) {
            if (next == start || next == end) continue

            val endToNext = minDistances[end][next]
            if (endToNext != Int.MAX_VALUE)
                updateRecursive(start, end, next, distance + endToNext, levels - 1)
        }

        return true
    }

    private fun initializeMinDistances() = Array(numberOfNodes) { IntArray(numberOfNodes) { Int.MAX_VALUE } }.apply {
        for (it in 0 until numberOfNodes) this[it][it] = 0

        edges.forEach { (l, r) ->
            this[l][r] = 1
            this[r][l] = 1
        }
    }

    private fun initializePaths() =
        Array(numberOfNodes) { Array(numberOfNodes) { emptyList<Mapping>() } }.apply {
            edges.forEach { edge ->
                val (l, r) = edge
                this[l][r] = listOf(edge)
                this[r][l] = listOf(edge)
            }
        }

    private fun initializeConnectionsMap() = (0 until numberOfNodes).map { node ->
        edges.mapNotNull { if (it.first == node) it.second else if (it.second == node) it.first else null }
    }
}

private fun sortedIndices(left: Int, right: Int) = min(left, right) to max(left, right)
