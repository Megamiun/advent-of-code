package br.com.gabryel.adventofcode.y2023.d09

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class MirageMaintenanceTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 9, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,114", "input,1666172641"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findNextNumberInNextLayer(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,2", "input,933"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { findPreviousNumberInNextLayer(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}