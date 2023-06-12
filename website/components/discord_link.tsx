import React from 'react'
import Link from 'next/link'

export interface DiscordLinkProps {
  linkText?: string
  className?: string
}

export const DiscordLink = (
  props: React.PropsWithChildren<DiscordLinkProps>
) => (
  <Link
    /* href="https://discord.gg/levelcrush" */ href="https://levelcrush.gg/discord"
    target="_blank"
    className={
      'block max-w-[12rem] text-center text-white bg-blue-600 hover:bg-blue-900 hover:cursor-pointer rounded px-4 py-2  mx-0 my-8 ' +
      (props.className || '')
    }
  >
    {props.linkText || 'Join us on Discord'}
    {props.children}
  </Link>
)

export default DiscordLink
