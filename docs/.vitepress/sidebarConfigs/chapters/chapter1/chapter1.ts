import type { DefaultTheme } from 'vitepress'
import { section1SidebarItems } from './sections/section1'

export const chapter1SidebarItems: DefaultTheme.SidebarItem[] = [
  {
    text: '第一部',
    items: [
      { text: 'はじめに', link: '/chapter1/index' },
      {
        text: '実習編',
        collapsed: true,
        items: [...section1SidebarItems]
      }
    ]
  }
]
