import React from 'react';
import Link from 'next/link';

/**
 * Controls the visual style of the button.
 */
export type ButtonIntention = 'normal' | 'attention' | 'danger' | 'inactive';

/**
 * provides properties for a <button> element + styling options
 */
export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  intention: ButtonIntention;
  href?: string;
  target?: string;
}

/**
 * Provides standard styling for normal <button> elements
 * @param props
 * @constructor
 */
export const Button = (props: ButtonProps) => (
  <button
    {...props}
    type="button"
    className={
      'block w-full px-4 py-2 rounded transition-all ' +
      (() => {
        switch (props.intention) {
          case 'normal':
            return 'bg-blue-600 hover:bg-blue-900 text-white hover:cursor-pointer';
          case 'attention':
            return 'bg-yellow-400 hover:bg-yellow-600 text-black hover:cursor-pointer';
          case 'danger':
            return 'bg-red-700 hover:bg-red-900 text-white hover:cursor-pointer ';
          case 'inactive':
            return 'bg-gray-600 hover:bg-gray-600 text-gray-900 hover:cursor-default';
          default:
            return '';
        }
      })() +
      ' ' +
      (props.className || '')
    }
  >
    {props.children}
  </button>
);

export const HyperlinkButton = (props: ButtonProps) => (
  <Link
    className={
      'block w-full px-4 py-2 rounded text-center transition-all ' +
      (() => {
        switch (props.intention) {
          case 'normal':
            return 'bg-blue-600 hover:bg-blue-900 text-white hover:cursor-pointer ';
          case 'attention':
            return 'bg-yellow-400 hover:bg-yellow-600 text-black hover:cursor-pointer ';
          case 'danger':
            return 'bg-red-700 hover:bg-red-900 text-white hover:cursor-pointer ';
          case 'inactive':
            return 'bg-gray-600 hover:bg-gray-600 text-gray-900 hover:cursor-default';
          default:
            return '';
        }
      })() +
      ' ' +
      (props.className || '')
    }
    href={props.href || ''}
    target={
      props.target || ((props.href || '').includes('http') ? '_blank' : '_self')
    }
  >
    {' '}
    {props.children}{' '}
  </Link>
);

export default Button;
