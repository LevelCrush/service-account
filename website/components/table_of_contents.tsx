import { faAngleDoubleRight } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import React from 'react';
import { Hyperlink } from '@website/components/elements/hyperlink';

// import the polyfill

export interface TableOfContentsNavigationItem {
  url: string;
  text: string;
  target?: '_self' | '_blank';
  subnavigation: TableOfContentsNavigationItem[];
}

export interface TableOfContentsProperties {
  navTree: TableOfContentsNavigationItem[];
}

/**
 * Lol this component needs to be revisted and done correctly. Only did a quick port from the original version
 */
export class TableOfContents extends React.Component<TableOfContentsProperties> {
  public constructor(props: any) {
    super(props);
  }

  public handleToggleCollapseClick(
    ev: React.MouseEvent<HTMLButtonElement, MouseEvent>
  ) {
    ev.preventDefault();

    const evTarget = ev.target as HTMLElement;
    if (evTarget.parentNode) {
      const evTargetParentNode = evTarget.parentNode as HTMLElement;
      if (
        evTarget.classList.contains('top-collapse') ||
        (evTargetParentNode &&
          evTargetParentNode.classList.contains('top-collapse'))
      ) {
        const entireTableOfContents = evTarget.closest('.table-of-contents');
        if (entireTableOfContents) {
          entireTableOfContents.classList.toggle('expanded');
        }
      } else {
        const closetLi = evTarget.closest('li');
        if (closetLi) {
          closetLi.click();
        }
      }
      if (evTarget) {
        evTarget.blur();
      }
    }
    return false;
  }

  public handleListItemClick(ev: React.MouseEvent<HTMLLIElement, MouseEvent>) {
    const evTarget = ev.target as HTMLElement;
    const evCurrentTarget = ev.currentTarget as HTMLElement;
    if (evTarget === evCurrentTarget) {
      if (evCurrentTarget.classList.contains('expanded')) {
        evCurrentTarget.classList.remove('expanded');
      } else {
        const expanded = document.querySelectorAll(
          '.table-of-contents li.expanded'
        );
        for (let i = 0; i < expanded.length; i++) {
          expanded[i].classList.remove('expanded');
        }
        evCurrentTarget.classList.add('expanded');
      }
    }
  }

  public handleLinkIntercept(
    ev: React.MouseEvent<HTMLAnchorElement, MouseEvent>
  ) {
    const element = ev.target as HTMLAnchorElement;
    const hashbangSplit = element.href.split('#');
    const hashBang = hashbangSplit.length > 0 ? hashbangSplit[1].trim() : '';

    if (hashBang.trim().length > 0) {
      ev.preventDefault();
      console.log(document.getElementById(hashBang));

      const hashBangElement = document.getElementById(hashBang);
      if (hashBangElement) {
        hashBangElement.scrollIntoView({
          behavior: 'smooth',
        });
      }
      if (element.parentNode) {
        const parentNode = element.parentNode as HTMLElement;
        if (parentNode.classList.contains('expanded') === false) {
          parentNode.click();
        }
      }
      return false;
    }
  }

  public render() {
    return (
      <div className="table-of-contents expanded group shadow-[0px_.4rem_0rem_1px_rgba(0,0,0,0.4)] flex-[1_1_auto] lg:flex-[0_1_30%] sticky top-[4.5rem] md:top-[5rem] self-start z-[98] transition-all duration-300 ease-in-out  dark:bg-slate-900 border-2 border-solid  dark:border-cyan-900 hover:dark:border-cyan-400  hover:border-gray-400 border-gray-200 bg-white  rounded-xl  rounded-t-none">
        <h2 className="transition-all duration-300 ease-in-out dark:bg-slate-900 border-b-2 border-solid  dark:border-cyan-900 dark:group-hover:border-cyan-400  group-hover:border-gray-400 border-gray-200">
          <button
            type="button"
            className="transition-all duration-300 px-4 py-2  inline-block text-left  w-full collapse-toggle top-collapse align-middle hover:bg-white hover:bg-opacity-10"
            onClick={this.handleToggleCollapseClick}
          >
            <span>Table of Contents</span>

            <FontAwesomeIcon
              icon={faAngleDoubleRight}
              className="float-right align-middle relative top-1"
            />
            <span className="text-xs float-right align-middle relative top-1 pr-4">
              (Click to show/hide)
            </span>
            <div className="clear-both"></div>
          </button>
        </h2>
        <nav aria-label="Table of Contents">
          <ol data-level="1">
            {this.props.navTree.map((navItem, index) => (
              <li
                key={'navitem_' + index}
                className="mb-2"
                onClick={this.handleListItemClick}
              >
                <Hyperlink
                  href={navItem.url}
                  target={navItem.target}
                  onClick={this.handleLinkIntercept}
                  className="text-sm"
                >
                  {navItem.text}
                </Hyperlink>
                <button
                  className="float-right inline-block w-auto collapse-toggle"
                  type="button"
                  onClick={this.handleToggleCollapseClick}
                >
                  <FontAwesomeIcon
                    icon={faAngleDoubleRight}
                    className="float-right align-middle relative top-1"
                  />
                </button>
                <nav aria-label={navItem.text + ' Sections'}>
                  <ol data-level="2">
                    {navItem.subnavigation.map((subItem, subIndex) => (
                      <li
                        className="mb-1 hover:cursor-default"
                        key={'navitem_' + index + '_' + subIndex}
                      >
                        <Hyperlink
                          className="text-sm"
                          href={subItem.url}
                          target={subItem.target}
                          onClick={this.handleLinkIntercept}
                        >
                          {subItem.text}
                        </Hyperlink>
                      </li>
                    ))}
                  </ol>
                </nav>
              </li>
            ))}
          </ol>
        </nav>
      </div>
    );
  }
}
