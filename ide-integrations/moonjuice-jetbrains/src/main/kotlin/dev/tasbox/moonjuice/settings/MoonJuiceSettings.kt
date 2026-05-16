package dev.tasbox.moonjuice.settings

import com.intellij.openapi.components.*
import com.intellij.configurationStore.Property

const val FROM_PATH_META_OPTION = "<From PATH>"

@Service
@State(name = "MoonJuiceSettings", storages = [Storage("moonjuice.xml")])
internal class MoonJuiceSettings :
  SerializablePersistentStateComponent<MoonJuiceSettingsState>(MoonJuiceSettingsState()) {
  companion object {
    fun getInstance(): MoonJuiceSettings = service()
  }

  var lspPath: String
    get() = state.lspPath
    set(value) {
      updateState {
        it.copy(lspPath = value)
      }
    }
}

internal data class MoonJuiceSettingsState(
  @JvmField @Property val lspPath: String = FROM_PATH_META_OPTION,
)
