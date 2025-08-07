package com.melvic.eanray.ui

import atlantafx.base.theme.{NordDark, PrimerDark}
import javafx.application.Application
import scalafx.application.JFXApp3

object Main extends JFXApp3:
  override def start(): Unit =
    Application.setUserAgentStylesheet(PrimerDark().getUserAgentStylesheet)

    stage = new JFXApp3.PrimaryStage:
      title = "Eanray"
      scene = new MainScene

      maximized = true