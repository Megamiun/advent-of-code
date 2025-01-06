package br.com.gabryel.adventofcode.y2023.d16

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class TheFloorWillBeLavaTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2023, 16, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,46", "input,7392"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getEnergizedTiles(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,51", "input,7665"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getMaxEnergizedTiles(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}