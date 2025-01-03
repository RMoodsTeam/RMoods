import React, { useState } from 'react';
import { Group, TextInput, Button, Popover, Checkbox, Box, NumberInput } from '@mantine/core';
import { DatePicker } from '@mantine/dates';
import { NlpAnalysisKind } from '../../../rmoods/types.ts';
import '@mantine/dates/styles.css';

interface FilterNavbarProps {
  filters: {
    title: string;
    startDate: Date | null;
    endDate: Date | null;
    nlpKinds: NlpAnalysisKind[];
    reportsPerPage: number;
  };
  onFilterChange: (field: string, value: any) => void;
  onClearFilters: () => void;
}

const FilterNavbar: React.FC<FilterNavbarProps> = ({ filters, onFilterChange, onClearFilters }) => {
  const [openPopover, setOpenPopover] = useState<string | null>(null);

  const handleNlpKindChange = (kind: NlpAnalysisKind, checked: boolean) => {
    onFilterChange('nlpKinds', checked
      ? [...filters.nlpKinds, kind]
      : filters.nlpKinds.filter(k => k !== kind));
  };

  return (
    <Box style={{ marginBottom: '15 px' }}>
      <Group justify='center'>
        <TextInput
          placeholder="Title"
          value={filters.title}
          onChange={(event) => onFilterChange('title', event.currentTarget.value)}
        />

        <Popover
          onClose={() => setOpenPopover(null)}
          position="bottom"
          withArrow
        >
          <Popover.Target>
            <Button onClick={() => setOpenPopover(openPopover === 'startDate' ? null : 'startDate')}>
              {filters.startDate ? filters.startDate.toLocaleDateString() : 'Pick Start Date'}
            </Button>
          </Popover.Target>
          <Popover.Dropdown>
            <DatePicker
              value={filters.startDate}
              onChange={(date) => {
                onFilterChange('startDate', date);
              }}
              minDate={new Date(2024, 11, 1)}
            />
          </Popover.Dropdown>
        </Popover>

        <Popover
          onClose={() => setOpenPopover(null)}
          position="bottom"
          withArrow
        >
          <Popover.Target>
            <Button>
              {filters.endDate ? filters.endDate.toLocaleDateString() : 'Pick End Date'}
            </Button>
          </Popover.Target>
          <Popover.Dropdown>
            <DatePicker
              value={filters.endDate}
              onChange={(date) => {
                onFilterChange('endDate', date);
                setOpenPopover(null);
              }}
            />
          </Popover.Dropdown>
        </Popover>

        <Popover
          onClose={() => setOpenPopover(null)}
          position="bottom"
          withArrow
        >
          <Popover.Target>
            <Button onClick={() => setOpenPopover(openPopover === 'nlpKinds' ? null : 'nlpKinds')}>
              {filters.nlpKinds.length > 0 ? `${filters.nlpKinds.length} NLP Kinds` : 'Pick NLP Kinds'}
            </Button>
          </Popover.Target>
          <Popover.Dropdown>
            {Object.values(NlpAnalysisKind).map(kind => (
              <Checkbox
                key={kind}
                label={kind}
                checked={filters.nlpKinds.includes(kind)}
                onChange={(event) => handleNlpKindChange(kind, event.currentTarget.checked)}
              />
            ))}
          </Popover.Dropdown>
        </Popover>
        <NumberInput
          placeholder="Reports per page"
          onChange={(value) => onFilterChange('reportsPerPage', value)}
          min={1}
        />
      </Group>
      <Group justify='center' style={{ marginTop: '20px' }}>
        <Button onClick={onClearFilters}>Clear Filters</Button>
      </Group>
    </Box>
  );
};

export default FilterNavbar;