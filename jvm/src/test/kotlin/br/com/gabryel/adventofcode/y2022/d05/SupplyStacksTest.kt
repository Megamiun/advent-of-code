package br.com.gabryel.adventofcode.y2022.d05

import br.com.gabryel.adventofcode.util.readGroupsOfLines
import br.com.gabryel.adventofcode.util.timed
import org.hamcrest.CoreMatchers.equalTo
import org.hamcrest.MatcherAssert.assertThat
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource

class SupplyStacksTest {
    private val files = listOf("sample", "input")
        .associateWith { readGroupsOfLines(2022, 5, it) }

    @ParameterizedTest(name = "Part 1 - {0}")
    @CsvSource(value = ["sample,CMZ", "input,SBPQRSCDF"])
    fun `Part 1`(file: String, expected: String) {
        val result = timed("Part 1 - $file") { restackOneByOne(files[file]!!) }

        assertThat(result, equalTo(expected))
    }

    @ParameterizedTest(name = "Part 2 - {0}")
    @CsvSource(value = ["sample,MCD", "input,RGLVRCQSB"])
    fun `Part 2`(file: String, expected: String) {
        val result = timed("Part 2 - $file") { restackFully(files[file]!!) }

        assertThat(result, equalTo(expected))
    }
}