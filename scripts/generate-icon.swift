import AppKit
import Foundation

// Draw every icon size directly, keeping the paddle silhouettes crisp in the Dock.
let outputDirectory = CommandLine.arguments.count > 1 ? CommandLine.arguments[1] : "."
let outputURL = URL(fileURLWithPath: outputDirectory)
let iconsetURL = outputURL.appendingPathComponent("Pong.iconset")
try FileManager.default.createDirectory(at: iconsetURL, withIntermediateDirectories: true)

func color(_ hex: Int) -> NSColor {
    NSColor(srgbRed: CGFloat((hex >> 16) & 255) / 255,
            green: CGFloat((hex >> 8) & 255) / 255,
            blue: CGFloat(hex & 255) / 255, alpha: 1)
}

func drawIcon(pixels: Int) -> Data {
    let bitmap = NSBitmapImageRep(bitmapDataPlanes: nil, pixelsWide: pixels, pixelsHigh: pixels,
        bitsPerSample: 8, samplesPerPixel: 4, hasAlpha: true, isPlanar: false,
        colorSpaceName: .deviceRGB, bytesPerRow: 0, bitsPerPixel: 0)!
    let context = NSGraphicsContext(bitmapImageRep: bitmap)!
    NSGraphicsContext.saveGraphicsState()
    NSGraphicsContext.current = context
    let transform = AffineTransform(scale: CGFloat(pixels) / 1024)
    (transform as NSAffineTransform).concat()

    let tile = NSBezierPath(roundedRect: NSRect(x: 64, y: 64, width: 896, height: 896),
                            xRadius: 200, yRadius: 200)
    NSGradient(starting: color(0x223554), ending: color(0x0B1020))!.draw(in: tile, angle: -90)
    color(0x334766).setStroke()
    tile.lineWidth = 6
    tile.stroke()

    // Court center line, with a gap around the ball.
    color(0x435471).setFill()
    for y in stride(from: 208, through: 784, by: 96) {
        NSBezierPath(roundedRect: NSRect(x: 505, y: y, width: 14, height: 48),
                     xRadius: 7, yRadius: 7).fill()
    }

    let left = NSBezierPath(roundedRect: NSRect(x: 222, y: 442, width: 64, height: 270),
                            xRadius: 32, yRadius: 32)
    NSGradient(starting: color(0x7DD3FC), ending: color(0x0EA5E9))!.draw(in: left, angle: -90)
    let right = NSBezierPath(roundedRect: NSRect(x: 738, y: 292, width: 64, height: 270),
                             xRadius: 32, yRadius: 32)
    NSGradient(starting: color(0xC4B5FD), ending: color(0x8B5CF6))!.draw(in: right, angle: -90)

    // A short trail gives the otherwise minimal mark a sense of motion.
    color(0x75849D).withAlphaComponent(0.5).setFill()
    NSBezierPath(ovalIn: NSRect(x: 398, y: 448, width: 38, height: 38)).fill()
    color(0xB0BBCD).withAlphaComponent(0.7).setFill()
    NSBezierPath(ovalIn: NSRect(x: 453, y: 477, width: 48, height: 48)).fill()
    color(0xF8FAFC).setFill()
    NSBezierPath(ovalIn: NSRect(x: 514, y: 506, width: 76, height: 76)).fill()
    NSGraphicsContext.restoreGraphicsState()
    return bitmap.representation(using: .png, properties: [:])!
}

for size in [16, 32, 128, 256, 512] {
    try drawIcon(pixels: size).write(to: iconsetURL.appendingPathComponent("icon_\(size)x\(size).png"))
    try drawIcon(pixels: size * 2).write(to: iconsetURL.appendingPathComponent("icon_\(size)x\(size)@2x.png"))
}
try drawIcon(pixels: 1024).write(to: outputURL.appendingPathComponent("Pong.png"))
