import Cocoa
import ApplicationServices

func getElementInfoAt(x: Float, y: Float) {
    let point = CGPoint(x: CGFloat(x), y: CGFloat(y))
    let sysWideElement = AXUIElementCreateSystemWide()
    var element: AXUIElement?
    let result = AXUIElementCopyElementAtPosition(sysWideElement, Float(point.x), Float(point.y), &element)
    guard result == .success, let elem = element else {
        print("{\"type\": \"Unknown\", \"text\": \"\"}")
        return
    }
    if !printElementInfo(elem) {
        var role: CFTypeRef?
        if AXUIElementCopyAttributeValue(elem, kAXRoleAttribute as CFString, &role) == .success,
           let roleStr = role as? String {
            print("{\"type\": \"\(roleStr)\", \"text\": \"\"}")
        } else {
            print("{\"type\": \"Unknown\", \"text\": \"\"}")
        }
    }
}

func printElementInfo(_ elem: AXUIElement) -> Bool {
    var role: CFTypeRef?
    if AXUIElementCopyAttributeValue(elem, kAXRoleAttribute as CFString, &role) == .success,
       let roleStr = role as? String {
        var title: CFTypeRef?
        if AXUIElementCopyAttributeValue(elem, kAXTitleAttribute as CFString, &title) == .success,
           let titleStr = title as? String, !titleStr.isEmpty {
            print("{\"type\": \"\(roleStr)\", \"text\": \"\(titleStr)\"}")
            return true
        }
        if AXUIElementCopyAttributeValue(elem, kAXValueAttribute as CFString, &title) == .success,
           let valueStr = title as? String, !valueStr.isEmpty {
            print("{\"type\": \"\(roleStr)\", \"text\": \"\(valueStr)\"}")
            return true
        }
        // 递归所有子元素
        var children: CFTypeRef?
        if AXUIElementCopyAttributeValue(elem, kAXChildrenAttribute as CFString, &children) == .success,
           let childrenArr = children as? [AXUIElement] {
            for child in childrenArr {
                if printElementInfo(child) {
                    return true // 只要有内容就立即返回
                }
            }
        }
        // 没找到内容
        // 只在最顶层才输出
        // print("{\"type\": \"\(roleStr)\", \"text\": \"\"}")
    }
    return false
}

let args = CommandLine.arguments
if args.count == 3, let x = Float(args[1]), let y = Float(args[2]) {
    getElementInfoAt(x: x, y: y)
} else {
    print("Usage: ax_query x y")
} 