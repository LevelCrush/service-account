import { searchInternalMembers } from '@ipc/service-destiny';
import {
  Button,
  ButtonProps,
  Input,
  InputProps,
  Radio,
  RadioGroup,
} from '@nextui-org/react';
import { useMemo, useState } from 'react';
import { twMerge } from 'tailwind-merge';

export type DestinySearchMode = 'member' | 'clan';

export interface DestinySearchBarProps {
  className?: string;
  variantSearchInput?: InputProps['variant'];
  variantSearchButton?: ButtonProps['variant'];
}

export const DestinySearchBar = (props: DestinySearchBarProps) => {
  const [isSearching, setIsSearching] = useState(false);
  const [searchMode, setSearchMode] = useState('member' as DestinySearchMode);
  const [searchValue, setSearchValue] = useState('');

  const validationState = useMemo(() => {
    if (searchValue.length === 0) {
      return undefined;
    }

    return (
      searchValue.trim().length > 0 ? 'valid' : 'invalid'
    ) as InputProps['validationState'];
  }, [searchValue]);

  return (
    <div className={twMerge('search-bar', props.className)}>
      <RadioGroup
        orientation="horizontal"
        value={searchMode}
        onValueChange={(v) => setSearchMode(v as DestinySearchMode)}
      >
        <Radio value="member">Member</Radio>
        <Radio value="clan">Clan</Radio>
      </RadioGroup>
      <div className="flex mt-4 gap-4">
        <Input
          isClearable={true}
          variant={props.variantSearchInput || 'bordered'}
          value={searchValue}
          label={`Search destiny for ${searchMode} information...`}
          validationState={validationState}
          onValueChange={(v) => setSearchValue(v)}
        />
        <Button
          isDisabled={searchValue.trim().length === 0}
          isLoading={isSearching}
          variant={props.variantSearchButton || 'ghost'}
          color={validationState === 'invalid' ? 'danger' : 'primary'}
          onClick={(ev) => {
            searchInternalMembers(searchValue, {
              page: 1,
              limit: 50,
            });
          }}
        >
          {isSearching ? 'Searching...' : 'Search'}
        </Button>
      </div>
    </div>
  );
};

export default DestinySearchBar;
