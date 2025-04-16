import Alert from 'react-bootstrap/Alert';

function AlertComponent() {
    return (
        <>
            {[
                'info',
            ].map((variant) => (
                <Alert key={variant} variant={variant}
                    style={{
                        color: "black",
                        background: "white",
                        border: "1px solid #ECECEC",
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