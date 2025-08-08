package com.melvic.eanray.ui

import scalafx.scene.{AmbientLight, Group, PerspectiveCamera, PointLight, SceneAntialiasing, SubScene}
import scalafx.scene.control.TitledPane
import scalafx.scene.paint.{Color, PhongMaterial}
import scalafx.scene.shape.{Box, Sphere, TriangleMesh}

class ObjectPalette extends TitledPane:
  text = "Available Objects"
  collapsible = true
  maxHeight = Double.MaxValue

  content = new ObjectListPane

  def makeTile(): SubScene =
    val tileGroup = new Group

    val shapes = List(
      List(Box(40, 40, 40), Sphere(25)),
      List(Box(40, 40, 40), Sphere(25))
    )

    val spacing = 100
    shapes.indices.foreach: row =>
      shapes(row).indices.foreach: col =>
        val shape = shapes(row)(col)
        shape.translateX = col * spacing
        shape.translateY = row * spacing
        shape.translateZ = 0

        shape.material = PhongMaterial(Color.Red)
        tileGroup.children.add(shape)

    val light = new PointLight(Color.White):
      translateX = 150
      translateY = -200
      translateZ = -300

    tileGroup.children.addAll(light, AmbientLight(Color.color(0.4, 0.4, 0.4)))

    val perspectiveCamera = new PerspectiveCamera(true):
      translateZ = -500
      nearClip = 0.1
      farClip = 10000

    new SubScene(tileGroup, 400, 300, true, SceneAntialiasing.Balanced):
      camera = perspectiveCamera


object ObjectPalette:
  private val HighIntensity: Int = (0.7 * 255).toInt
  private val LowIntensity: Int = (0.3 * 255).toInt