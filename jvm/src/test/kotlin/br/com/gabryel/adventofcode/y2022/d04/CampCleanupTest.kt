package br.com.gabryel.adventofcode.y2022.d04

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class CampCleanupTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2022, 4, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,2", "input,582"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getFullyContainedRanges(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,4", "input,893"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getOverlappingRanges(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}