package br.com.gabryel.adventofcode.y2023.d08

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class HauntedWastelandTest {
    private val files = listOf("sample", "sample-2", "sample-3", "input")
        .associateWith { readLines(2023, 8, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,2", "sample-2,6", "input,19783"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findStepsToArrive(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample-3,6", "input,9177460370549"])
    fun `Part 2`(file: String, expected: Long) {
        val result = timed("Part 2 - $file") { findGhostStepsToArrive(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}