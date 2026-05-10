package dev.tasbox.moonjuice

import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor

class MoonJuiceLspServerDescriptor(project: Project) : ProjectWideLspServerDescriptor(project, "MoonJuice") {
  companion object {
    fun isSupportedFile(file: VirtualFile): Boolean = file.fileType === MoonJuiceFileType
  }

  override fun isSupportedFile(file: VirtualFile): Boolean = MoonJuiceLspServerDescriptor.isSupportedFile(file)

  // TODO: Make path configurable or bundle with plugin
  override fun createCommandLine(): GeneralCommandLine =
    GeneralCommandLine("~/.cargo/bin/moonjuice-lsp")
}
