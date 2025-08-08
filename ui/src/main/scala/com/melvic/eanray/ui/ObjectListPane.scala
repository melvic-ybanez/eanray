package com.melvic.eanray.ui

import com.melvic.eanray.ui.ObjectListPane.{
  makeCubeTile,
  makeDiskTile,
  makeParallelogramTitle,
  makeSphereTile,
  makeTriangleTile,
  makeVolumeTile
}
import scalafx.geometry.{Insets, Pos}
import scalafx.scene.canvas.{Canvas, GraphicsContext}
import scalafx.scene.control.Label
import scalafx.scene.layout.{TilePane, VBox}
import scalafx.scene.paint.Color

class ObjectListPane extends TilePane:
  hgap = 25
  vgap = 25
  prefColumns = 3
  tileAlignment = Pos.Center
  padding = Insets(25)

  children ++= Seq(
    makeCubeTile,
    makeSphereTile,
    makeTriangleTile,
    makeDiskTile,
    makeParallelogramTitle,
    makeVolumeTile
  )

object ObjectListPane:
  private val HighIntensity: Int = (0.7 * 255).toInt
  private val LowIntensity: Int = (0.3 * 255).toInt

  def makeShapeTile(name: String)(drawIcon: GraphicsContext => Unit): VBox =
    val canvas = Canvas(70, 70)

    val gc = canvas.graphicsContext2D
    drawIcon(gc)

    val tile = new VBox:
      children ++= Seq(canvas, new Label(name))
      alignment = Pos.Center

    tile

  def makeCubeTile: VBox = makeShapeTile("Cube"): gc =>
    val x = 10
    val y = 20
    val size = 40
    val angle = Math.toRadians(45)
    val dx = Math.cos(angle) * size // cah
    val dy = Math.sin(angle) * size // soh

    // reddish front
    gc.fill = Color.rgb(HighIntensity, LowIntensity, LowIntensity)
    gc.fillPolygon(
      Array(x, x + size, x + size, x),
      Array(y, y, y + size, y + size),
      4
    )

    // green-ish right
    gc.fill = Color.rgb(LowIntensity, HighIntensity, LowIntensity)
    gc.fillPolygon(
      Array(x + size, x + size + dx, x + size + dx, x + size),
      Array(y, y - dy, y + size - dy, y + size),
      4
    )

    // blue-ish top
    gc.fill = Color.rgb(LowIntensity, LowIntensity, HighIntensity)
    gc.fillPolygon(
      Array(x, x + dx, x + size + dx, x + size),
      Array(y, y - dy, y - dy, y),
      4
    )

  def makeSphereTile: VBox = makeShapeTile("Sphere"): gc =>
    val size = 60
    gc.fill = Color.rgb(HighIntensity, LowIntensity, LowIntensity)
    gc.fillOval(8, 2, size, size)

  def makeTriangleTile: VBox = makeShapeTile("Triangle"): gc =>
    val x = 8
    val y = 2
    val size = 60
    gc.fill = Color.rgb(LowIntensity, HighIntensity, LowIntensity)
    gc.fillPolygon(Array(x + size / 2, x + size, x), Array(y, y + size, y + size), 3)

  def makeDiskTile: VBox = makeShapeTile("Disk"): gc =>
    gc.fill = Color.rgb(LowIntensity, LowIntensity, HighIntensity)
    gc.fillOval(15, 2, 40, 60)

  def makeParallelogramTitle: VBox = makeShapeTile("Parallelogram"): gc =>
    val x = 8
    val y = 9
    val hSide = 55
    val vSide = 47
    val offset = 7

    gc.fill = Color.rgb(HighIntensity, HighIntensity, LowIntensity)
    gc.fillPolygon(
      Array(x + offset, x + hSide + offset, x + hSide, x),
      Array(y, y, y + vSide, y + vSide),
      4
    )

  def makeVolumeTile: VBox = makeShapeTile("Volume"): gc =>
    val size = 12
    val highIntensity = (HighIntensity * 0.8).toInt
    gc.fill = Color.rgb(highIntensity, LowIntensity, highIntensity)

    gc.fillOval(8, 10, size, size)
    gc.fillOval(25, 5, size, size)
    gc.fillOval(42, 16, size, size)
    gc.fillOval(27, 25, size, size)
    gc.fillOval(10, 30, size, size)
    gc.fillOval(45, 35, size, size)
    gc.fillOval(50, 8, size, size)
    gc.fillOval(33, 45, size, size)
