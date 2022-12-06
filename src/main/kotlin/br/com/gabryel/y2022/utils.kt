package br.com.gabryel.y2022

import java.util.Scanner
import kotlin.io.path.Path
import kotlin.io.path.writeText

private val scanner = Scanner(System.`in`)

fun getLines() = getLines { it }

fun <T> getLines(transform: (line: String) -> T) =
    generateSequence { if (scanner.hasNextLine()) scanner.nextLine().ifEmpty { null } else null }
        .mapNotNull(transform)

fun main() {
    (1..7).forEach { day ->
        val fill = day.toString().padStart(2, '0')

        Path("/home/gabryel/git/advent-of-code/advent-of-code/.run/2022/$fill.run.xml").writeText(
            """
                <component name="ProjectRunConfigurationManager">
                  <configuration default="false" name="2022/${fill}" type="JetRunConfigurationType">
                    <option name="MAIN_CLASS_NAME" value="br.com.gabryel.y2022.Day${fill}Kt" />
                    <module name="advent-of-code.main" />
                    <option name="REDIRECT_INPUT" value="true" />
                    <option name="INPUT_FILE" value="${"PROJECT_DIR"}${'$'}/src/main/resources/input/2022/day${fill}" />
                    <shortenClasspath name="NONE" />
                    <method v="2">
                      <option name="Make" enabled="true" />
                    </method>
                  </configuration>
                </component>
            """.trimIndent()
        )
    }
}