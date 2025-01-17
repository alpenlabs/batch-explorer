import Alert from 'react-bootstrap/Alert';

function AlertComponent() {
    return (
        <>
            {[
                'info',
            ].map((variant) => (
                <Alert key={variant} variant={variant}
                    style={{
                        color: "var(--strata-terracotta-med)",
                        background: "var(--strata-sand-med)",
                        border: "1px solid var(--strata-terracotta-med)",
                        //width 30%
                        width: "30%",
                        margin: "10px ",
                    }}>
                    Please enter a valid range
                </Alert>
            ))}
        </>
    );
}

export default AlertComponent;