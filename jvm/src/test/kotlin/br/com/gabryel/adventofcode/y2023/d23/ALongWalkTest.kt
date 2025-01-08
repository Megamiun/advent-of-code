package br.com.gabryel.adventofcode.y2023.d23

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class ALongWalkTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 23, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,94", "input,2050"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { findLongestWalk(files[file]!!, false) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,154", "input,6262"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { findLongestWalk(files[file]!!, true) }

        assertThat(result, equalTo(expected))
    }
}