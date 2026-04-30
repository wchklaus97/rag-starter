import SwiftUI

public enum DesignTokens {
    public enum Primitive {
        public static let ink950 = Color(red: 17 / 255, green: 24 / 255, blue: 39 / 255)
        public static let ink700 = Color(red: 55 / 255, green: 65 / 255, blue: 81 / 255)
        public static let ink500 = Color(red: 107 / 255, green: 114 / 255, blue: 128 / 255)
        public static let paper50 = Color(red: 249 / 255, green: 250 / 255, blue: 251 / 255)
        public static let paper100 = Color(red: 243 / 255, green: 244 / 255, blue: 246 / 255)
        public static let line200 = Color(red: 229 / 255, green: 231 / 255, blue: 235 / 255)
        public static let blue600 = Color(red: 37 / 255, green: 99 / 255, blue: 235 / 255)
        public static let blue700 = Color(red: 29 / 255, green: 78 / 255, blue: 216 / 255)
        public static let amber600 = Color(red: 217 / 255, green: 119 / 255, blue: 6 / 255)
        public static let green600 = Color(red: 22 / 255, green: 163 / 255, blue: 74 / 255)
        public static let red600 = Color(red: 220 / 255, green: 38 / 255, blue: 38 / 255)
    }

    public enum Semantic {
        public static let background = Primitive.paper50
        public static let surface = Primitive.paper100
        public static let border = Primitive.line200
        public static let textPrimary = Primitive.ink950
        public static let textSecondary = Primitive.ink700
        public static let textMuted = Primitive.ink500
        public static let accent = Primitive.blue600
        public static let accentPressed = Primitive.blue700
        public static let success = Primitive.green600
        public static let warning = Primitive.amber600
        public static let danger = Primitive.red600
    }

    public enum Spacing {
        public static let xs: CGFloat = 4
        public static let s: CGFloat = 8
        public static let m: CGFloat = 12
        public static let l: CGFloat = 16
        public static let xl: CGFloat = 24
    }

    public enum Radius {
        public static let small: CGFloat = 8
        public static let medium: CGFloat = 12
        public static let large: CGFloat = 18
    }

    public enum Component {
        public enum MessageBubble {
            public static let userBackground = Semantic.accent
            public static let assistantBackground = Semantic.surface
            public static let radius = Radius.medium
            public static let padding = Spacing.m
        }

        public enum StatusPill {
            public static let ready = Semantic.success
            public static let loading = Semantic.warning
            public static let error = Semantic.danger
            public static let radius = Radius.large
            public static let horizontalPadding = Spacing.m
            public static let verticalPadding = Spacing.xs
        }
    }
}
