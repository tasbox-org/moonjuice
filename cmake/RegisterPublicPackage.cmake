cmake_minimum_required(VERSION 3.28)

include(GNUInstallDirs)

function(RegisterPublicPackage PACKAGE_NAME)
  message(STATUS "Registering public package '${PACKAGE_NAME}'")

  # Traditionally, you shouldn't GLOB source files as it breaks changed file detection.
  # However, the "new" CONFIGURE_DEPENDS parameter solves this for most cases with minimal build-time overhead
  file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS "packages/${PACKAGE_NAME}/*.cpp")

  add_library(${PACKAGE_NAME} OBJECT)
  target_sources(
          ${PACKAGE_NAME} PUBLIC
          FILE_SET MODULES TYPE CXX_MODUELS FILES ${SOURCES}
  )

  install(
          TARGETS ${PACKAGE_NAME}
          EXPORT ${PACKAGE_NAME}
          LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
          ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
          RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
          FILE_SET HEADERS
  )
  install(
          EXPORT ${PACKAGE_NAME}
          DESTINATION share/${PACKAGE_NAME}
          FILE ${PACKAGE_NAME}Config.cmake
          NAMESPACE MoonJuice::
  )
endfunction()
