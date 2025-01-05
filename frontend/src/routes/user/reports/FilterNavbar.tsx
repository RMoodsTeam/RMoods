import React from 'react';
import {
  ActionIcon,
  Box,
  Card,
  Center,
  Grid,
  Group,
  MultiSelect,
  Select,
  TextInput,
} from '@mantine/core';
import { DatePickerInput } from '@mantine/dates';
import { NlpAnalysisKind } from '../../../rmoods/types.ts';
import '@mantine/dates/styles.css';
import { IconFilterCancel } from '@tabler/icons-react';

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

const FilterNavbar: React.FC<FilterNavbarProps> = ({
  filters,
  onFilterChange,
  onClearFilters,
}) => {
  const handleNlpKindChange = (kind: NlpAnalysisKind, checked: boolean) => {
    onFilterChange(
      'nlpKinds',
      checked
        ? [...filters.nlpKinds, kind]
        : filters.nlpKinds.filter((k) => k !== kind)
    );
  };

  return (
    <Box>
      <Card>
        <Center>
          <Box style={{ marginBottom: '15 px', width: '50em' }}>
            <Grid justify="center" grow>
              <Grid.Col span={4}>
                <TextInput
                  placeholder="eg. My report"
                  label="Title pattern"
                  value={filters.title}
                  onChange={(event) =>
                    onFilterChange('title', event.currentTarget.value)
                  }
                />
              </Grid.Col>
              <Grid.Col span={4}>
                <DatePickerInput
                  label="From date"
                  placeholder="eg. 2024-12-01"
                  value={filters.startDate}
                  clearable
                  onChange={(date) => {
                    onFilterChange('startDate', date);
                  }}
                  minDate={new Date(2024, 11, 1)}
                />
              </Grid.Col>
              <Grid.Col span={4}>
                <DatePickerInput
                  label="To date"
                  placeholder="eg. 2024-12-31"
                  value={filters.endDate}
                  clearable
                  onChange={(date) => {
                    onFilterChange('endDate', date);
                  }}
                />
              </Grid.Col>
              <Grid.Col span={4}>
                <MultiSelect
                  wrapperProps={{ style: { width: '100%' } }}
                  data={Object.values(NlpAnalysisKind)}
                  label="NLP Analyses"
                  value={filters.nlpKinds}
                  onChange={(value) =>
                    onFilterChange('nlpKinds', value as NlpAnalysisKind[])
                  }
                />
              </Grid.Col>
            </Grid>
          </Box>
        </Center>
      </Card>
      <Group justify="right" style={{ marginTop: '20px' }} align={'end'}>
        <ActionIcon onClick={onClearFilters} size={'lg'}>
          <IconFilterCancel />
        </ActionIcon>
        <Select
          label="Per page"
          placeholder="Reports per page"
          onChange={(value) =>
            onFilterChange('reportsPerPage', Number.parseInt(value!, 10))
          }
          w={'5em'}
          data={[10, 20, 50, 100].map((value) => value.toString())}
        />
      </Group>
    </Box>
  );
};

export default FilterNavbar;
