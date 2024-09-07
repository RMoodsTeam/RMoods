import React from 'react'

/**
 * Section component, used to display a section with a title and content.
 * @param title - The title of the section.
 * @param content - The content of the section.
 * @returns Element
 */
interface SectionProps {
    title: string
    content: string
}

const Section: React.FC<SectionProps> = ({ title, content }) => {
    return (
        <div className='pt-10 flex flex-wrap items-center justify-between'>
            <div className='w-full'>
                <h3 className='text-xl font-semibold text-center mb-2'>{title}</h3>
                <p className='text-left'>{content}</p>
            </div>
        </div>
    )
}

export default Section