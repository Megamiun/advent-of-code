package br.com.gabryel.adventofcode.y2022.d07

import br.com.gabryel.adventofcode.util.readLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class NoSpaceLeftOnDeviceTest {
    private val files = listOf("sample", "input")
        .associateWith { readLines(2022, 7, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,95437", "input,1501149"])
    fun `Part 1`(file: String, expected: Int) {
        val result = timed("Part 1 - $file") { getSummaryOfDirsLessThan(files[file]!!, 100000) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,24933642", "input,10096985"])
    fun `Part 2`(file: String, expected: Int) {
        val result = timed("Part 2 - $file") { getSummaryOfDirsBetween(files[file]!!, 30000000, 70000000) }

        assertThat(result, equalTo(expected))
    }
}