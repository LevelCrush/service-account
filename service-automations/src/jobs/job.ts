export interface Job {
    run: () => Promise<void>;
    cleanup: () => Promise<void>;
}

export default Job;
